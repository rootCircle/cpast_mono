#!/usr/bin/perl
use strict;
use warnings;

# my $s = '7 4 5 6 7 8 9 10';
# my $s =~ /(\d+)\s+((??{"(\\s*\\d+){$^N}"}))/;

# print "$1\n$2\n"

print "hi NAME\n";



sub check_pattern {
    my ($pattern, $string) = @_;

    # Replace the pattern syntax with regex expressions
    $pattern =~ s/SPACE/ /g;
    $pattern =~ s/N/([+-]?\\d+)/g;
    $pattern =~ s/F/([+-]?\\d+[\.\\d+]?)/g;
    $pattern =~ s/S/(\\w+)/g;
    $pattern =~ s/C/(\\w)/g;
    # $pattern =~ s/\(([^)]+)\)/"($1)"/eg;
    # $pattern =~ s/\(([^)]+)\){(\d+)}/"($1){$2}"/eg;
    # $pattern =~ s/(\d+)\|(\d+)/"{" . $1 . "," . $2 . "}"/eg;

    # Replace the back-references with the corresponding capturing group references
    # $pattern =~ s/\\(\d+)/"\\$1"/eg;

    # Construct the final regular expression
    my $regex = qr/^$pattern$/;

    print $regex;

    # Check if the string matches the pattern
    if ($string =~ $regex) {
        return 1; # String matches the pattern
    } else {
        return 0; # String does not match the pattern
    }
}

my $pattern = '(N) N[,1000] (?:N F S){\1}';
my $string = '2 2 2 2.2 ABC2 3 4.5 ASD';

if (check_pattern($pattern, $string)) {
    print "Pattern matched!\n";
} else {
    print "Pattern did not match!\n";
}

