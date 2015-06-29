require 'benchmark'

Benchmark.bm do |x|
  i = 4
  x.report {
    100000000.times do
      i = i + 1
    end
  }
end
