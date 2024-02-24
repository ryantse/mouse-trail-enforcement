package main

import (
	"flag"
	"fmt"
	"time"
	"unsafe"

	"golang.org/x/sys/windows"
)

func main() {
	var (
		user32DLL                 = windows.NewLazyDLL("user32.dll")
		procSystemParametersInfoW = user32DLL.NewProc("SystemParametersInfoW")
		null                      = uintptr(0)
		mouseTrailsCurrent        = new(uint32)
		mouseTrailsTarget         = flag.Uint("target", 2, "target mouse trails value")
		debug                     = flag.Bool("debug", false, "debug mode")
	)

	flag.Parse()

	if *debug {
		fmt.Printf("Monitoring for mouse trail changes (target = %d).\n", *mouseTrailsTarget)
	}

	for {
		procSystemParametersInfoW.Call(uintptr(0x005E) /*SPI_GETMOUSETRAILS*/, null, uintptr(unsafe.Pointer(mouseTrailsCurrent)), null)

		if uint(*mouseTrailsCurrent) != *mouseTrailsTarget {
			if *debug {
				fmt.Printf("Detected mouse trail change (current = %d, target = %d).\n", *mouseTrailsCurrent, *mouseTrailsTarget)
			}
			procSystemParametersInfoW.Call(uintptr(0x005D) /*SPI_SETMOUSETRAILS*/, uintptr(*mouseTrailsTarget), null, null)
		}

		time.Sleep(1 * time.Second)
	}
}
