; Gain control using POT0
; POT0 controls the volume from 0 to 100%

; Read left ADC input
RDAX ADCL, 1.0

; Multiply by POT0 for volume control
MULX POT0

; Write to left DAC output
WRAX DACL, 0.0
