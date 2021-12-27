package priorityqueue

import (
	"container/heap"
)

type Node struct {
	X             int
	Y             int
	TotalDistance int
	heapIndex     int
}

type PriorityQueue struct {
	pq  *internalPriorityQueue
}

func CreatePriorityQueue() PriorityQueue {
	return PriorityQueue{new(internalPriorityQueue)}
}

func (pq PriorityQueue) Len() int {
	return len(*pq.pq)
}

func (pq *PriorityQueue) Add(node *Node) {
	heap.Push(pq.pq, node)
}

func (pq *PriorityQueue) RemoveMin() *Node {
	return heap.Pop(pq.pq).(*Node)
}

func (pq *PriorityQueue) Update(node *Node, totalDistance int) {
	node.TotalDistance = totalDistance
	heap.Fix(pq.pq, node.heapIndex)
}

type internalPriorityQueue []*Node

func (pq internalPriorityQueue) Len() int {
	return len(pq)
}

func (pq internalPriorityQueue) Less(i, j int) bool {
	return pq[i].TotalDistance < pq[j].TotalDistance
}

func (pq internalPriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].heapIndex = i
	pq[j].heapIndex = j
}

func (pq *internalPriorityQueue) Push(x interface{}) {
	n := len(*pq)
	node := x.(*Node)
	node.heapIndex = n
	*pq = append(*pq, node)
}

func (pq *internalPriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	node := old[n-1]
	old[n-1] = nil  // avoid memory leak
	node.heapIndex = -1 // for safety
	*pq = old[0 : n-1]
	return node
}
