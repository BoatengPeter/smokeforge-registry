package server

import "net/http/pprof"

var (
	pprofIndex   = pprof.Index
	pprofCmdline = pprof.Cmdline
	pprofProfile = pprof.Profile
	pprofSymbol  = pprof.Symbol
	pprofTrace   = pprof.Trace
)
