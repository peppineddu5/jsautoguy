import ffi from "ffi-napi"
import ref from "ref-napi"
import ArrayType from "ref-array-napi";

const IntArray = ArrayType(ref.types.int)
const lib = ffi.Library('./target/debug/jsautogui', {
    getMousePos:["bool",[IntArray]],
    setMousePos:["bool",["int32","int32"]]
  });
  
const cordinate =new IntArray(2);

lib.getMousePos(cordinate);
console.log(cordinate.toArray())
lib.setMousePos(0,0)

for (let i = 0; i < 200; i++) {
  lib.setMousePos(1.2*i,1.2*i)
}