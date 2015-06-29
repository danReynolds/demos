require 'ffi'
require 'benchmark'

module Hello
  extend FFI::Library
  begin
    ffi_lib 'target/debug/libincrement.so'
  rescue Exception
    ffi_lib 'target/release/libincrement.so'
  end
  attach_function :increment, [ :int ], :int
end

Benchmark.bm do |x|
  x.report {
    input = 4
    output = Hello.increment(input)
  }
end
