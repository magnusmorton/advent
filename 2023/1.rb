require "numbers_in_words"

digits = /one|two|three|four|five|six|seven|eight|nine|\d/

text = STDIN.read

puts text.lines.map { | line |
    nums = line.gsub(/[A-Za-z]/, '').chomp
    "#{nums[0]}#{nums[-1]}".to_i()
}.sum

puts text.lines.map { | line |
    nums = line.chomp
    scan = nums.scan(digits)
    # to_i necessary because of bug in numbers_in_words when given a digit
    (NumbersInWords.in_numbers(scan[0]).to_i().to_s + NumbersInWords.in_numbers(scan[-1]).to_i().to_s).to_i
}.sum