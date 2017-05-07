#![crate_type = "staticlib"]

use std::ptr;
use std::mem;

unsafe fn memset<T>(s: *mut T, c: u8, n: u8)
{
  let s = s as *mut u8;
  let c = c as u8;

  for i in 0..n as isize 
  {
    ptr::write_volatile(s.offset(i), c);
  }
}

#[no_mangle]
pub unsafe extern "C" fn rle_encode(src: *mut u8, src_len: usize, dst: *mut u8, dst_len: usize, bytes_written: *mut usize) -> i32
{
  let mut src_end = src.offset(src_len as isize);
  let mut dst_end = dst.offset(dst_len as isize);
  let mut d = dst;
  let mut s = src;

  while s < src_end
  {
    if (dst_end as usize - d as usize) < 2
    { 
      return -1;
    }

    let mut val = *s;
    let mut run = 0;

    while *s == val && (s as usize) < (src_end as usize) && run < 255
    {
      s = s.offset(1);
      run += 1;
    }

    *d = run;
    d = d.offset(1);
    *d = val;
    d = d.offset(1);
  }
  *bytes_written = d as usize - dst as usize;
  0
}

#[no_mangle]
pub unsafe extern "C" fn rle_decode(src: *mut u8, src_len: usize, dst: *mut u8, dst_len: usize, bytes_written: *mut usize) -> i32
{
  if (src_len & 1) != 0
  {
    return -1;
  }

  let mut src_end = src.offset(src_len as isize);
  let mut dst_end = dst.offset(dst_len as isize);
  let mut d = dst;
  let mut s = src;

  while s < src_end
  {
    let mut run = *s;
    let mut val = *s.offset(1);

    if (dst_end as u8 - d as u8) < run
    {
      return -1;
    }

    memset(d, val, run);
    d = d.offset(run as isize);
    s = s.offset(2);
  }
  *bytes_written = d as usize - dst as usize;
  0
}
