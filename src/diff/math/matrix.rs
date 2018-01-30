  
 

    use std::ops::{Index,IndexMut};


    ///Matrix for tracing difference durign single comparison
    pub struct Matrix<T>
        where T : Clone
    {
        pub r_cnt     : usize,
        pub c_cnt     : usize,     
        pub r_tot_cnt : usize, 
        pub c_tot_cnt : usize,  
        data     : Vec<T>
    }

   

    impl<T> Matrix<T> 
        where T : Clone + Default
    {

        pub fn new(row_count : usize, col_count: usize) -> Matrix<T>
        {           
           let _m = Matrix::<T> {  
                r_cnt       : row_count,  
                c_cnt       : col_count, 
                r_tot_cnt   : row_count,
                c_tot_cnt   : col_count,
                data: vec![Default::default(); row_count * col_count] 
            };          

           _m
        }
      
        #[inline]
        pub fn set_size(&mut self,  rcnt : usize,  ccnt : usize)
        {
            self.r_cnt = rcnt;
            self.c_cnt = ccnt;
        }      

        pub fn fill_with_val(&mut self, val : T)
        {
            let size = self.r_cnt * self.c_cnt;
            for i in 0..size {
                self.data[i] = val.clone();
            }
        }

        
        #[inline]
        fn get_index(&self,  row_idx : usize, col_idx : usize) -> usize
        {           
           row_idx * self.c_cnt + col_idx           
        }


    }

     impl<T> Index<(usize, usize)> for Matrix<T>
         where T : Clone + Default
     {
            type Output = T;
           
            fn index(&self, (i, j): (usize, usize)) -> &T 
            {
                assert!(i < self.r_cnt * self.c_cnt && j < self.c_cnt);
                let idx = self.get_index(i,j);
                &self.data[idx]
            }
     } 

     impl<T> IndexMut<(usize, usize)> for Matrix<T> 
        where T : Clone + Default
     {
      
        fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T 
        {            
            assert!(i < self.r_cnt * self.c_cnt && j < self.c_cnt);
            let idx = self.get_index(i,j);
            &mut self.data[idx]         
        }
    }
 
