    use std::ops::{Index,IndexMut};


    ///Matrix for tracing difference during single comparison
    pub struct Matrix<T>       
    {
        pub r_cnt     : usize,
        pub c_cnt     : usize,     
        pub r_tot_cnt : usize, 
        pub c_tot_cnt : usize,  
        data     : Vec<T>
    }

   

    impl<T> Matrix<T>         
    {

        pub fn new(row_count : usize, col_count: usize) -> Matrix<T>
            where T : Default
        {           
           let mut _m = Matrix::<T> {  
                r_cnt       : row_count,  
                c_cnt       : col_count, 
                r_tot_cnt   : row_count,
                c_tot_cnt   : col_count,
                data        : Vec::new(),          
            };          

            //init with default value
            _m.data = Vec::with_capacity(row_count * col_count);
            for _ in 0.._m.data.capacity() {
                _m.data.push(Default::default());
            }
           _m
        }
      
        #[inline]
        pub fn set_size(&mut self,  rcnt : usize,  ccnt : usize)
        {
            self.r_cnt = rcnt;
            self.c_cnt = ccnt;
        }      

        pub fn fill_with_val(&mut self, val : T)
                where T: Clone
        {
            let size = self.r_cnt * self.c_cnt;
            for i in 0..size {
                self.data[i] = val.clone();
            }
        }
    }

     impl<T> Index<(usize, usize)> for Matrix<T>        
     {          
           type Output = T;
           
            fn index(&self, (i, j): (usize, usize)) -> &T 
            {
                assert!(i < self.r_cnt * self.c_cnt && j < self.c_cnt);               
                &self.data[i * self.c_cnt + j]
            }
     } 

     impl<T> IndexMut<(usize, usize)> for Matrix<T>         
     {      
        fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T 
        {            
            assert!(i < self.r_cnt * self.c_cnt && j < self.c_cnt);          
            &mut self.data[i * self.c_cnt + j]         
        }
    }
 
