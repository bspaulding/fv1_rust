; Simple pass-through program
; Copies left input directly to left output

; Read left ADC input with unity gain
RDAX ADCL, 1.0

; Write to left DAC output
WRAX DACL, 0.0
