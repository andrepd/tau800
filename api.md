# Frontend ↔ Backend API Specification

This document outiles the conventions for interaction between the frontend
(Electron) and backend (Rust), and sequence of events in operation.

## API

## Sequence of Operation

```
							 Backend

							 ┌─────┐
							 │Start│
							 └──┬──┘
						 ┌──────────┴────────────┐
					   ┌─────┤ Threaded poll response├──┐
					   │     │     and processing    │  │
	    Frontend                       │     └───────────────────────┘  │
					   │                                │
	     ┌─────┐                       │                       ┌────────┴───────┐
	     │Start│                       │                       │Begin processing├───┐
	     └──┬──┘                       │                       │    bytecode    │   │
		│                          │                       └────────┬───────┘   │
		│                          │                                │           │
		│                          │                                │           │
	   ┌────┴─────┐                ┌───┴────────┐               ┌───────┴──────┐    │
	┌──┤State Poll├──────────────► │Request poll│       ┌───────┤Pending query?│    │
	│  └────┬─────┘                │            │       │       └───────┬──────┘    │
	│       │                      │     .      │◄──────┘               │           │
	   ┌────┴─────────┐  response  │     .      │                       │           │
	▲  │Await Response│◄───────────┤     .      │                       └───────────┘
	│  └────┬─────────┘            └────────────┘
	│       │
	│ ┌─────┴──────────────────┐
	└─┤Update visual state     │
	  │to reflect current state│
	  └────────────────────────┘
```

## Poll Object

The Electron side expects, upon poll, an object with the following properties:

```
{
	"number_register_bits": {
		
	}
}
```
