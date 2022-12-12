package main

// tag::fast[]

// This is an implementation using an A* algorithm implemented by me.

import (
	"fmt"
	"log"

	"github.com/razziel89/astar"
)

func abs(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

func fastAlgorithm(nodes astar.GraphOps, start, end *astar.Node) (int, bool) {
	// Build heuristic that measures distance to end node.
	heuristic := astar.ConstantHeuristic{}
	// Extract payload. We know that these are the coordinates.
	endVec := end.Payload.(Vec)
	for node := range *nodes.(*astar.Graph) {
		vec := node.Payload.(Vec)
		// This estimate hopefully pulls us towards the goal.
		estimateX := vec.x - endVec.x
		estimateY := vec.y - endVec.y
		estimate := abs(estimateX + estimateY)
		err := heuristic.AddNode(node, estimate)
		if err != nil {
			log.Fatal(err.Error())
		}
	}
	// Plot path.
	path, err := astar.FindPath(nodes, start, end, heuristic.Heuristic(0))
	if err != nil {
		fmt.Println(err.Error())
		return 0, false
	}
	// Compute distance.
	dist := 0
	for _, node := range path[1:] {
		fmt.Println(node.ToString())
		dist++
	}
	fmt.Println("Distance is", dist)
	return dist, true
}

// end::fast[]
