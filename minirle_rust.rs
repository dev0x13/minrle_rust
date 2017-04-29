#![crate_type = "lib"]
#![crate_name = "minirle_rust"]

use std::ptr;
use std::mem;

// fn test() -> &'static str
// {
//   let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
//   pangram
// }

#[allow(dead_code)]
fn rle_encode (mut src: &mut [u8], src_len: usize, dst: &mut [u8], dst_len: usize, bytes_written: &[usize]) -> i32
{
  let mut src_pointer: *const u8 = src.as_mut_ptr();
  let src_end: *const u8 = unsafe { src_pointer.offset(src_len as isize) };
  let mut dst_pointer: *mut u8 = dst.as_mut_ptr();
  let dst_end: *const u8 = unsafe { dst_pointer.offset(dst_len as isize) };

  while src_pointer < src_end
  {
    if (dst_end as usize - dst_pointer as usize) < 2
    { 
      return -1;
    }

    let val: *const u8 = src_pointer;
    let mut run: u8 = 0;

    while src_pointer == val && (src_pointer as usize) < (src_end as usize) && run < 255
    {
      src_pointer = unsafe { src_pointer.offset(1) };
      run += 1;
    }

    dst_pointer = unsafe { dst_pointer.offset(1) };
    unsafe { ptr::write_bytes(dst_pointer, run, mem::size_of::<u8>()) };

    dst_pointer = unsafe { dst_pointer.offset(1) };
    unsafe { ptr::write_bytes(dst_pointer, run, mem::size_of::<*const u8>()) };
      
  }
  0
}

#[allow(dead_code)]
fn rle_decode (src: &[u8], src_len: usize, dst: &[u8], dst_len: usize, bytes_written: &[usize]) -> i32
{
  0
}

// fn main()
// {
//   let x: &'static str;
//   x = test();
//   println!("{}", x);
// }