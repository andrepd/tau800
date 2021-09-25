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
	numbers: [ <"hours">, <"digits"> ],
	registers: [ [array of boolean values] for each register index ],
	stack: <value from 0 to 6 indicating how full the stack is>,
	history: [ <last 6 commands as strings> ]
}
```
