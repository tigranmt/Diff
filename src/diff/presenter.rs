use std;
use std::io::Write;

use diff::difflib::DiffCell;
use diff::difflib::DiffOperation;


pub struct DiffPresenter    
{   
    writer : Box<std::io::Write>,
}

///Built-in impementation of DiffPresenter
impl DiffPresenter    
{

    pub fn new (_w : Box<std::io::Write>) -> DiffPresenter        
    {
         DiffPresenter{ writer : _w }        
    }

    pub fn header(&mut self, _old : &str, _new : &str)-> std::result::Result<usize, std::io::Error>
    {
        let mut bytes_written = self.writer.write("/***********************************************************\n".as_bytes())?;
        bytes_written += self.writer.write("* Difference between : \n".as_bytes())?;
        bytes_written += self.writer.write(format!("* {} \n", _old).as_bytes())?;
        bytes_written += self.writer.write("* - and - \n".as_bytes())?;
        bytes_written += self.writer.write(format!("* {} \n", _new).as_bytes())?;
        bytes_written += self.writer.write("***********************************************************/\n".as_bytes())?;

        Ok(bytes_written)
    }

    pub fn present(&mut self, _old : &str, _new : &str, line_num : u32,  result : &[DiffCell]) -> std::result::Result<usize, std::io::Error>
    {
        let mut bytes_written = 0;
        if result.len() > 0 
        {
            let red_diff_count = self.skip_redundant(&result);
            let iter = result.iter().skip(red_diff_count);
            for diff in iter
            {
                bytes_written = self.writer.write("\n".as_bytes())?;
                bytes_written += self.writer.write(format!("Line: {}, ==={}===\n", line_num, diff.operation).as_bytes())?;
                
                if diff.operation == DiffOperation::Insert  
                {                
                    bytes_written += self.present_insert(_old, _new, &diff)?;        
                }
                else if diff.operation == DiffOperation::Remove
                {                          
                    bytes_written += self.present_remove(_old, _new, &diff)?;
                }  
                else if diff.operation == DiffOperation::Update
                {                
                    bytes_written += self.present_udate(_old, _new, &diff)?;
                }  
            }
        }

        Ok(bytes_written)
    }

    ///Skips redundant diff cells, by ignoring them 
    fn skip_redundant(&self, vec : &[DiffCell]) -> usize
    {
        let mut count_to_skip : usize = 0;
        for i in  0 .. vec.len() - 1
        {
            let prev = &vec[i];
            let next = &vec[i + 1];

            if (prev.operation == DiffOperation::Insert && next.operation == DiffOperation::Remove)
                    || (prev.operation == DiffOperation::Remove && next.operation == DiffOperation::Insert)
            {
                if prev.start == next.start && prev.count == next.count
                {
                    count_to_skip += 2; //skip both
                }

            }
        }

        count_to_skip
    }

    ///Presents INSERT change 
    fn present_insert(&mut self, _old : &str, _new : &str, diff : &DiffCell) -> std::result::Result<usize, std::io::Error>
    {
      
        let out_new = _new.chars().skip(diff.start).take(diff.count).collect::<String>();
        let mut bytes_written = self.writer.write(format!("{}{}\n", _old, out_new).as_bytes())?;

        let empty_before_change = std::iter::repeat(" ").take(_old.len()).collect::<String>(); 
        if diff.count > 1 
        { 
            let empty_change = std::iter::repeat(" ").take(diff.count - 2).collect::<String>(); //first pipe and last one have to be skiped  
            bytes_written += self.writer.write(format!("{}|{}|\n", empty_before_change, empty_change).as_bytes())?;
        }
        else {
            bytes_written += self.writer.write(format!("{}|\n", empty_before_change).as_bytes())?;
        }
        
        let ins = std::iter::repeat("+").take(diff.count).collect::<String>();         
        bytes_written += self.writer.write(format!("{}{}\n", empty_before_change, ins).as_bytes())?;      
        
        Ok(bytes_written)
    }

    ///Presents REMOVE change
    fn present_remove(&mut self, _old : &str, _new : &str, diff : &DiffCell) -> std::result::Result<usize, std::io::Error>
    {
        let mut bytes_written = self.writer.write(format!("{}\n", _old).as_bytes())?;

        let empty_before_change = std::iter::repeat(" ").take(diff.start).collect::<String>(); 
        if diff.count > 1 
        { 
            let empty_change = std::iter::repeat(" ").take(diff.count - 2).collect::<String>(); //first pipe and last one have to be skiped  
            bytes_written += self.writer.write(format!("{}|{}|\n", empty_before_change, empty_change).as_bytes())?;
        }
        else {
            bytes_written += self.writer.write(format!("{}|\n", empty_before_change).as_bytes())?;
        }
        
        let dels = std::iter::repeat("x").take(diff.count).collect::<String>(); 
        bytes_written += self.writer.write(format!("{}{}\n", empty_before_change, dels).as_bytes())?;       

        Ok(bytes_written)       
    }


    ///Presents UPDATE change
    fn present_udate(&mut self, _old : &str, _new : &str, diff : &DiffCell) -> std::result::Result<usize, std::io::Error>
    {
        let out_new  = _new.chars().skip(diff.start).take(diff.count).collect::<String>();
        let mut bytes_written = self.writer.write(format!("{}\n", _old).as_bytes())?;

        
        let empty_before_change = std::iter::repeat(" ").take(diff.start).collect::<String>();          
        if diff.count > 1 
        { 
            let empty_change = std::iter::repeat(" ").take(diff.count - 2).collect::<String>(); //first pipe and last one have to be skiped 
            bytes_written += self.writer.write(format!("{}|{}|\n", empty_before_change, empty_change).as_bytes())?;
        }
        else 
        {
            bytes_written += self.writer.write(format!("{}|\n", empty_before_change).as_bytes())?;
        }
        
        bytes_written += self.writer.write(format!("{}{}\n", empty_before_change, out_new).as_bytes())?;
        
        Ok(bytes_written)
    }

}



