// Generated by `wit-bindgen` 0.8.0. DO NOT EDIT!
package testfs

// #include "testfs.h"
import "C"

// Export functions from testfs
var testfs Testfs = nil
func SetTestfs(i Testfs) {
  testfs = i
}
type Testfs interface {
  Openfile() 
}
//export testfs_openfile
func TestfsOpenfile() {
  testfs.Openfile()
  
}
