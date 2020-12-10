use strict;
use Data::Dumper;
my @lines = <STDIN>;

my @numbers = map(int,@lines) ;


@numbers = sort { $a <=> $b } @numbers;
my $jolts = 0;
my %diffs;
foreach (@numbers) {
	my $diff = $_ - $jolts;
	$diffs{$diff}++;
	$jolts = $_;
}

$diffs{3}++;
print Dumper %diffs;

print $diffs{1} * $diffs{3} + '\n';
