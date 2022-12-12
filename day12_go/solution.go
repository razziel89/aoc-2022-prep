// Package main is the main executable for razziel89's solution for this day's advent.
package main

import (
	"fmt"
	"log"
	"sort"

	"github.com/razziel89/astar"
)

// tag::solution[]

const (
	numNeigh = 4
	replicas = 5
)

//nolint:funlen
func gridToNodes(grid Grid) (astar.GraphOps, *astar.Node, *astar.Node, error) {
	result := astar.NewGraph(0)
	var start, end *astar.Node
	// Remember which node belonged to which location.
	convertedNodes := map[Vec]*astar.Node{}

	// Process the nodes themselves.
	for vec, cost := range grid {
		if _, ok := convertedNodes[vec]; ok {
			err := fmt.Errorf("node already present")
			return result, start, end, err
		}

		realCost := cost
		if cost == startPlaceholder {
			// The starting node.
			realCost = 0
		} else if cost == endPlaceholder {
			// The end node.
			realCost = int('z' - 'a')
		}
		grid[vec] = realCost

		node, err := astar.NewNode(fmt.Sprint(vec), realCost, numNeigh, vec)
		if err != nil {
			return result, start, end, err
		}

		convertedNodes[vec] = node
		result.Add(node)

		if cost == startPlaceholder {
			start = node
		}
		if cost == endPlaceholder {
			end = node
		}
	}
	if start == nil || end == nil {
		err := fmt.Errorf("cannot find start or end node in grid")
		return result, start, end, err
	}
	// Add pairwise connections to those nodes with a hight that is at most 1 higher. This might
	// take a while.
	for vec := range grid {
		node, ok := convertedNodes[vec]
		if !ok {
			err := fmt.Errorf("node not found")
			return result, start, end, err
		}
		for neigh := range pointEnv(vec) {
			if !grid.Has(neigh) {
				continue // Ignore nodes outside the grid.
			}
			neighNode, ok := convertedNodes[neigh]
			if !ok {
				err := fmt.Errorf("node not found")
				return result, start, end, err
			}
			if grid[neigh]-grid[vec] <= 1 {
				node.AddConnection(neighNode)
			}
		}
	}
	return result, start, end, nil
}

// nolint: funlen
func main() {
	grid, err := ReadLinesAsGrid()
	if err != nil {
		log.Fatal(err.Error())
	}
	possibleStarts := []Vec{}
	var end Vec
	for node, val := range grid {
		if val == 0 {
			possibleStarts = append(possibleStarts, node)
		}
		if val == endPlaceholder {
			end = node
		}
	}
	// Part 1.
	nodes, startNode, endNode, err := gridToNodes(grid)
	if err != nil {
		log.Fatal(err.Error())
	}

	fastAlgorithm(nodes, startNode, endNode)

	// Part 2.

	distances := []int{}

	for idx, start := range possibleStarts {
		grid[start] = startPlaceholder
		grid[end] = endPlaceholder
		fmt.Println(start, end, idx, len(possibleStarts))

		nodes, startNode, endNode, err = gridToNodes(grid)
		if err != nil {
			log.Fatal(err.Error())
		}

		dist, found := fastAlgorithm(nodes, startNode, endNode)
		if found {
			distances = append(distances, dist)
		}
	}

	sort.Ints(distances)
	fmt.Println(distances)
}

// end::solution[]
