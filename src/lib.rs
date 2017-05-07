#![crate_type = "lib"]
#![crate_name = "minirle_rust_lib"]

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

unsafe fn rle_encode(src: *mut u8, dst: *mut u8, src_len: usize, dst_len: usize, bytes_written: *mut usize) -> i32
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

    ptr::write_bytes(d, run, mem::size_of::<u8>());
    d = d.offset(1);
    ptr::write_bytes(d, run, mem::size_of::<*const u8>());
    d = d.offset(1);
  }
  *bytes_written = d as usize - dst as usize;
  0
}

unsafe fn rle_decode(src: *mut u8, dst: *mut u8, src_len: usize, dst_len: usize, bytes_written: *mut usize) -> i32
{
  let mut src_end = unsafe { src.offset(src_len as isize) };
  let mut dst_end = unsafe { dst.offset(dst_len as isize) };
  let mut d = dst;
  let mut s = src;

  if (src_len & 1) != 0
  {
    return -1;
  }

  while s < src_end
  {
    let mut run = *s;
    let mut val = *s.offset(1);

    if (dst_end as u8 - d as u8) < run
    {
      return -1;
    }

    d = d.offset(run as isize);
    memset(d, val, run);
  }
  *bytes_written = d as usize - dst as usize;
  0
}