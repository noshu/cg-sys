//CG+ binding
use libc::{c_double,c_int};

extern "C"{
    pub fn cgfam_(n:*const c_int,x:*mut c_double,f:*const c_double,g:*const c_double,d:*mut c_double,
    gold:*mut c_double,iprint:*const c_int,eps:*const c_double,w:*mut c_double,iflag:&mut c_int,irest:*const c_int,
    method:*const c_int,finish:*const c_int);
}
