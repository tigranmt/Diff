//! Main module for diff implementaiton 
//! Contains public structure StringDiff
//! to use to access complete feature set 
//! of a libriary    
//! Example: 
//!    
//!   let l    = String::from("Hello world");
//!   let r    = String::from("Hello wrld");
//!   let sd   = StringDiff::new(); 
//!   let diff = sd.get_diff(&l,&r);
//!
//!
//!
use diff::math::Matrix;
use std::cmp::min;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Display;


//helper method
#[cfg(debug_assertions)]
#[allow(dead_code)]
fn print_matrix<T>(mtx : &Matrix<T>)
    where T : Clone + Default + Display
{
    for row in 0..mtx.r_cnt
    {
        for col in 0..mtx.c_cnt
        {
            print!("{}-", mtx[(row, col)]);
        }

        println!("");
    }
}

/// Cell that discribes single occuried difference between strings.
/// start: Start index of character in the left string provide 
/// count: Count of consecutive characters affected by the _same_ difference 
/// operation: Difference operation  
#[derive(Debug)]
pub struct DiffCell
{
    pub start : usize,
    pub count : usize,
    pub operation : DiffOperation
}


/// Types of operation required to execute on current character, or range of characters 
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum DiffOperation 
{
    Insert,
    Remove,    
    Update,
    None
}

impl Default for DiffOperation
{
    fn default() -> DiffOperation {DiffOperation::None}
}

impl Display for DiffOperation
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            DiffOperation::Insert =>  write!(f, "(i)"),
            DiffOperation::Remove =>  write!(f, "(r)"),
            DiffOperation::Update =>  write!(f, "(u)"),
            _      =>  write!(f, "(-)"),
        }
       
    }
}


pub struct StringDiff
{
    mtx : Matrix<u32>,
    directions : Matrix<DiffOperation>
}   


impl StringDiff
{
    pub fn new() -> StringDiff
    {           
       let matrix : Matrix<u32>  = Matrix::new(512, 512);     
       let dirs : Matrix<DiffOperation> = Matrix::new(512, 512);
       StringDiff { mtx : matrix, directions: dirs}
    }


    /// Returns a vector of differences occured in 2 strings provided as parameters
    pub fn get_diff(&mut self, _old : &str, _new : &str) -> Vec<DiffCell>
    {            

        if _old == _new 
        {
            return vec![];
        }

        let mut row_count       =  _old.chars().count() + 1;  //space for 0s
        let mut column_count    =  _new.chars().count() + 1; //space for 0s
        
        //trim strings to equal size
        let mut v   = Vec::with_capacity(512);
        if row_count > column_count {
            v.push(DiffCell{start: column_count - 1, count: row_count - column_count, operation: DiffOperation::Remove});
            row_count = column_count;
        }
        else if column_count > row_count {
            v.push(DiffCell{start: row_count - 1, count: column_count - row_count, operation: DiffOperation::Insert});
            column_count = row_count; 
        }

       
        if row_count == 1 && column_count > 1
        {
            return vec![DiffCell{start: 0, count: column_count, operation: DiffOperation::Insert}];  
        }
        else if row_count > 1 && column_count == 1
        {
            return vec![DiffCell{start: 0, count: column_count, operation: DiffOperation::Remove}]; 
        }
       
        
        //init bigger matrix
        if row_count > self.mtx.r_tot_cnt || column_count > self.mtx.c_tot_cnt
        {               
            self.mtx        =  Matrix::new(row_count, column_count);
            self.directions =  Matrix::new(row_count, column_count);
        }
        else {
            //set row and column count to the current, requred size of source strings
            self.mtx.set_size(row_count, column_count);
            self.directions.set_size(row_count, column_count);
           
        }
       
      
        self.compile_matrix(&_old, &_new);           
        self.fill_result(&mut v);        

        v
    }


    ///Resets matrix to its original state, filled with 0s
    fn init_matrix(&mut self)
    {    
        self.mtx.fill_with_val(0); 

        //first row  0..col_count
        let col_count = self.mtx.c_cnt;
        for c in  0 .. col_count
        {
           self.mtx[(0, c)] = c as u32;  
        }  

        //first column 0.. row_count
        let row_count = self.mtx.r_cnt;        
        for r in  0 .. row_count 
        {
           self.mtx[(r, 0)] = r as u32;  
        }

    }

    ///Get concrete direction from the cell of the Matrix
    fn get_curdir(&self, i : usize, j : usize) -> DiffOperation
    {              
        if self.mtx[(i+1, j+1)] == self.mtx[(i, j)] + 1 {
            DiffOperation::Update //update                        
        }
        else if self.mtx[(i+1, j+1)] == self.mtx[(i, j+1)] + 1 {                     
            DiffOperation::Remove //remove                
        }
        else if self.mtx[(i+1, j+1)] == self.mtx[(i+1, j)] + 1 {                         
             DiffOperation::Insert //insert                
        }   
        else {
            DiffOperation::None
        }              
              
    }    

    
    /// Naive implementation of Eugene W. Myer's string diff algorithm 
    /// Paper source: http://www.xmailserver.org/diff2.pdf
    fn compile_matrix(&mut self, _old : &str, _new : &str) 
    {          
        self.init_matrix();
        
        for (i, o_ch) in _old.chars().enumerate().take(self.mtx.r_cnt-1) 
        {               
            for (j, n_ch) in _new.chars().enumerate().take(self.mtx.c_cnt-1) 
            {     
                if o_ch == n_ch {           
                    //No change            
                    self.mtx[(i+1, j+1)] =  self.mtx[(i, j)];        
                    self.directions[(i+1, j+1)]= DiffOperation::None;                                                                                          
                }
                else {                    
                    //Change 
                    self.mtx[(i+1, j+1)] =  min(self.mtx[(i, j)], min(self.mtx[(i, j+1)], self.mtx[(i+1, j)])) + 1;                
                    self.directions[(i+1, j+1)] =  self.get_curdir(i,j);  
                }
            }
        }
        
    }

  
    /// Compiles vector of differences computed from the matrix
    fn fill_result(&self, v : &mut Vec<DiffCell>)
    {      
        
        let mut row = self.mtx.r_cnt-1;
        let mut col = self.mtx.c_cnt-1;

        
        while row > 0 && col > 0
        {          
            let mut op_count = 0;          
            let mut op = DiffOperation::None;
                    

            while  row > 0 && col > 0 && 
                        self.directions[(row, col)] == DiffOperation::None  //no change
            {
                row -=1;
                col -=1;         
            }           

            while row > 0 && col > 0 
                    && self.directions[(row, col)] == DiffOperation::Update //update
            {                       
                row = row - 1;
                col = col - 1;     
                op_count = op_count+1;        
                op = DiffOperation::Update;             
            }

            if op != DiffOperation::None
            {
                v.push(DiffCell{start: row, count: op_count, operation: op.clone()});
                op_count = 0;
                op = DiffOperation::None;
            }

            while row > 0 && col > 0 
                    && self.directions[(row, col)] == DiffOperation::Remove //remove
            {                                                                                  
                row = row - 1;     //go UP
                op_count = op_count+1;  
                op = DiffOperation::Remove;                                
            }

            if op != DiffOperation::None
            {
                v.push(DiffCell{start: row, count: op_count, operation: op.clone()});
                op_count = 0;
                op = DiffOperation::None;
            }

            while row > 0 && col > 0 
                    && self.directions[(row, col)] == DiffOperation::Insert //insert
            {                                                                             
                col = col - 1;     //go LEFT
                op_count = op_count+1; 
                op = DiffOperation::Insert;                        
            }

            if op != DiffOperation::None 
            {
                v.push(DiffCell{start: row, count: op_count, operation: op.clone()});               
            }

        }      
    }

    
}

    
#[cfg(test)]
mod tests 
{   
    
    use ::diff::difflib::*;

    #[test]
    fn empty_strings() 
    {
       let _old   = String::from("");
       let _new   = String::from("");       
       let mut sd   = StringDiff::new();        
       let res = sd.get_diff(&_old, &_new);
       assert_eq!(res.len() == 0, true);
    }

    #[test]
    fn equal_strings() 
    {
       let _old   = String::from("Hello world");
       let _new   = String::from("Hello world");
       let mut sd   = StringDiff::new(); 
       let res = sd.get_diff(&_old, &_new);
       assert_eq!(res.len() == 0, true);
    }


    #[test]
    fn swap_first_characters() 
    {
       let _old   = String::from("Hleol world");
       let _new   = String::from("Hello world");
       let mut sd   = StringDiff::new(); 
       let vec = sd.get_diff(&_old, &_new);      

       assert_eq!(vec.len(), 3);
       
       assert_eq!(vec[0].start, 4);
       assert_eq!(vec[0].count, 1);
       assert_eq!(vec[0].operation, DiffOperation::Remove);

       assert_eq!(vec[1].start, 2);
       assert_eq!(vec[1].count, 1);
       assert_eq!(vec[1].operation, DiffOperation::Update);

       assert_eq!(vec[2].start, 1);
       assert_eq!(vec[2].count, 1);
       assert_eq!(vec[2].operation, DiffOperation::Insert);
    }

    #[test]
    fn swap_characters() 
    {
       let _old   = String::from("Hello world");
       let _new   = String::from("Hello wordl");
       let mut sd   = StringDiff::new(); 
       let vec = sd.get_diff(&_old, &_new);     

       assert_eq!(vec.len(), 1);
       assert_eq!(vec[0].start, 9);
       assert_eq!(vec[0].count, 2);
       assert_eq!(vec[0].operation, DiffOperation::Update);
    }


    #[test]
    fn remove_to_empty()
    {
       let _old   = String::from("H");
       let _new   = String::from("");
       let mut sd   = StringDiff::new(); 
       let vec = sd.get_diff(&_old, &_new);      

       println!("vec: {:?}", vec);
       
       assert_eq!(vec.len(), 1);

       assert_eq!(vec[0].start, 0);
       assert_eq!(vec[0].count, 1);
       assert_eq!(vec[0].operation, DiffOperation::Remove);    
    }

    #[test]
    fn add_to_empty()
    {
       let _old   = String::from("");
       let _new   = String::from("Hello");
       let mut sd   = StringDiff::new(); 
       let vec = sd.get_diff(&_old, &_new);      

       println!("vec: {:?}", vec);
       
       assert_eq!(vec.len(), 1);

       assert_eq!(vec[0].start, 0);
       assert_eq!(vec[0].count, 5);
       assert_eq!(vec[0].operation, DiffOperation::Insert);    
    }

    #[test]
    fn add_character() 
    {
       let _old   = String::from("Hello worl");
       let _new   = String::from("Hello world");
       let mut sd   = StringDiff::new(); 
       let vec = sd.get_diff(&_old, &_new);
     
       
       assert_eq!(vec.len(), 1);
       assert_eq!(vec[0].start, 10);
       assert_eq!(vec[0].count, 1);
       assert_eq!(vec[0].operation, DiffOperation::Insert);
    }

    #[test]
    fn remove_multiple_characters() 
    {
       let _old   = String::from("Hello world");
       let _new   = String::from("H");
       let mut sd   = StringDiff::new(); 
       let vec = sd.get_diff(&_old, &_new);
       
      
       assert_eq!(vec.len(), 1);
       assert_eq!(vec[0].start, 1);
       assert_eq!(vec[0].count, 10);
       assert_eq!(vec[0].operation, DiffOperation::Remove);
    }


    #[test]
    fn update_remove()
    {
       let _old   = String::from("Hallo");
       let _new   = String::from("Hello world");
       let mut sd  = StringDiff::new(); 
       let vec = sd.get_diff(&_old, &_new);      

     
       assert_eq!(vec.len(), 2);      
       assert_eq!(vec[0].start, 5);
       assert_eq!(vec[0].count, 6);
       assert_eq!(vec[0].operation, DiffOperation::Insert);
            
      
       assert_eq!(vec[1].start, 1);
       assert_eq!(vec[1].count, 1);
       assert_eq!(vec[1].operation, DiffOperation::Update);
    }


    #[test]
    fn update_add()
    {
       let _old   = String::from("Ha#%o      xxxx");
       let _new   = String::from("Hello world");
       let mut sd   = StringDiff::new(); 
       let vec = sd.get_diff(&_old, &_new);
       

       println!("vec: {:?}", vec);
       
       assert_eq!(vec.len(), 3);

       assert_eq!(vec[0].start, 11);
       assert_eq!(vec[0].count, 4);
       assert_eq!(vec[0].operation, DiffOperation::Remove);
            
       assert_eq!(vec[1].start, 6);
       assert_eq!(vec[1].count, 5);
       assert_eq!(vec[1].operation, DiffOperation::Update);

       assert_eq!(vec[2].start, 1);
       assert_eq!(vec[2].count, 3);
       assert_eq!(vec[2].operation, DiffOperation::Update);
    }

    #[test]
    fn remove_ch()
    {
        let _old = String::from("你好，世界");
        let _new = String::from("你好");
        let mut sd   = StringDiff::new(); 
        let vec = sd.get_diff(&_old, &_new);

        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0].start, 2);
        assert_eq!(vec[0].count, 3);
        assert_eq!(vec[0].operation, DiffOperation::Remove);

    }

    #[test]
    fn update_arm()
    {
        let _old = String::from("դա կատարյալ է");
        let _new = String::from("դա կատարյալ  ");     
        let mut sd   = StringDiff::new(); 
        let vec = sd.get_diff(&_old, &_new);

        assert_eq!(vec.len(), 1);
        assert_eq!(vec.len(), 1);
        assert_eq!(vec[0].start, 12);
        assert_eq!(vec[0].count, 1);
        assert_eq!(vec[0].operation, DiffOperation::Update);

    }
}
