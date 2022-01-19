[目標] 同じアセンブリコードを入力として、jitと素のbinaryでどっちが速いのかをくらべたい
* 比較としては
  * asmをrjitに喰わせて実行させた際の実行時間
  * asmを一旦compileしてマシンコードにしたものを、実行させた際の実行時間
  を比較する
* asmをgccとかでcompileしてしまうとcrtとかが入って正確な比較ができないので、独自のcrt(`crt.s`)をスタートアップとしてlinkする.(makefile参照)


### Flow
1. read .s file
2. mmap (mem alloc for generated code.)
3. initialize asm instance
4. loop
   1. assemble from .s, and generate machine code.
   2. run ↑ code.
5. terminate