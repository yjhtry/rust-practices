package main

import (
	"fmt"
)

type Tree[T any] struct {
	Left  *Tree[T]
	Right *Tree[T]
	Value T
}

func (root *Tree[T]) walk(c chan T) {
	queue := []*Tree[T]{root}

	for len(queue) > 0 {
		node := queue[0]

		c <- node.Value

		if node.Left != nil {
			queue = append(queue, node.Left)
		}

		if node.Right != nil {
			queue = append(queue, node.Right)
		}

		queue = queue[1:]
	}

	close(c)

}

func RunTreeDemo() {
	root := Tree[int]{
		Left: &Tree[int]{
			Value: 2,
		},
		Right: &Tree[int]{
			Value: 3,
		},
		Value: 1,
	}

	c := make(chan int)

	go root.walk(c)

	for v := range c {
		fmt.Println(v)
	}

}

func sum(list []int, ch chan int) {
	res := 0
	for _, v := range list {
		res += v
	}

	ch <- res
}

func RunSumDemo() {
	list := make([]int, 10000)

	for i := 0; i < 10000; i++ {
		list[i] = i
	}

	ch := make(chan int)

	for i := 0; i < 10000; i += 100 {
		end := i + 100
		go sum(list[i:end], ch)

	}

	res := 0

	for i := 1; i < 100; i++ {
		res += <-ch
	}

	fmt.Println(res)

}
