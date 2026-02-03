; Simple delay/echo effect
; A basic echo with fixed delay time
; POT1 controls feedback amount
; POT2 controls wet/dry mix

; Read input
RDAX ADCL, 1.0
WRAX REG0, 0.0          ; Save input to REG0

; Read from delay line at address 4000
RDA 4000, 0.5           ; Read delayed signal

; Add feedback
MULX POT1               ; Scale by feedback amount
RDAX REG0, 1.0          ; Add input
WRA 0, 0.0              ; Write to delay line at address 0

; Mix wet/dry
; ACC now has delayed signal
MULX POT2               ; Scale by wet amount
RDAX REG0, 1.0          ; Add dry signal

; Output
WRAX DACL, 0.0
