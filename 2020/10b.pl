use strict;
use Data::Dumper;
my @lines = <STDIN>;

my @numbers = map(int,@lines) ;


@numbers = sort { $a <=> $b } @numbers;
my $jolts = 0;
my %configs;
$configs{0} = 1;
my @prev = (0);
my $number;
my $max = 0;
foreach $number (@numbers) {
	@prev = grep { ( $number - $_) < 4} @prev;
	my $ways = 0;
	foreach (@prev) {
		$ways += $configs{$number};
	}
	push @prev, $number;
	$max = $number;
	$configs{$number} = $ways;
}
print "$max \n";
print $configs{$max};

