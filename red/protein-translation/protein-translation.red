Red [
	description: {"Protein Translation" exercise solution for exercism platform}
	author: "" ; you can write your name here, in quotes
]

proteins: function [
	strand [string!]
] [
	chain: copy []
  tr: [
		"AUG" "Methionine"
		"UUU" "Phenylalanine" "UUC" "Phenylalanine"              
        "UUA" "Leucine" "UUG" "Leucine"
        "UCU" "Serine" "UCC" "Serine" "UCA" "Serine" "UCG" "Serine"
		"UAU" "Tyrosine" "UAC" "Tyrosine"
        "UGU" "Cysteine" "UGC" "Cysteine"
		"UGG" "Tryptophan"
		"UAA" "STOP" "UAG" "STOP" "UGA" "STOP"
	]
    foreach [a b c] strand [
        codon: rejoin [a b c]
				probe codon
        protein: select tr codon
        if protein = "STOP" [ break ]
        append chain protein
    ]
    chain
]


