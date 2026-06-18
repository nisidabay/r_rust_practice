.PHONY: check clean

EXERCISES = ex01_rectangle ex02_traffic_light ex03_elevator ex04_shapes

check: $(EXERCISES)
	@echo "All exercises passed!"

ex01_rectangle: ex01_rectangle.rs
	rustc --edition 2021 ex01_rectangle.rs -o /tmp/$@ && /tmp/$@

ex02_traffic_light: ex02_traffic_light.rs
	rustc --edition 2021 ex02_traffic_light.rs -o /tmp/$@ && /tmp/$@

ex03_elevator: ex03_elevator.rs
	rustc --edition 2021 ex03_elevator.rs -o /tmp/$@ && /tmp/$@

ex04_shapes: ex04_shapes.rs
	rustc --edition 2021 ex04_shapes.rs -o /tmp/$@ && /tmp/$@

clean:
	rm -f /tmp/ex01_rectangle /tmp/ex02_traffic_light /tmp/ex03_elevator /tmp/ex04_shapes
