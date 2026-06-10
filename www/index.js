import * as sim from "lib-simulation-wasm";

CanvasRenderingContext2D.prototype.drawTriangle = function (
	x,
	y,
	size,
	rotation,
) {
	this.beginPath();
	this.moveTo(
		x - Math.sin(rotation) * size * 1.5,
		y + Math.cos(rotation) * size * 1.5,
	);
	this.lineTo(
		x - Math.sin(rotation + (2.0 / 3.0) * Math.PI) * size,
		y + Math.cos(rotation + (2.0 / 3.0) * Math.PI) * size,
	);
	this.lineTo(
		x - Math.sin(rotation + (4.0 / 3.0) * Math.PI) * size,
		y + Math.cos(rotation + (4.0 / 3.0) * Math.PI) * size,
	);

	this.lineTo(
		x - Math.sin(rotation) * size * 1.5,
		y + Math.cos(rotation) * size * 1.5,
	);

	this.fillStyle = "rgb(255, 255, 255)";
	this.fill();
};

CanvasRenderingContext2D.prototype.drawCircle = function (x, y, radius) {
	this.beginPath();

	this.arc(x, y, radius, 0, 2 * Math.PI);

	this.fillStyle = "rgb(0, 255, 128)";
	this.fill();
};

const viewport = document.getElementById("viewport");
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;

const viewportScale = window.devicePixelRatio || 1;

viewport.height = viewportHeight * viewportScale;
viewport.width = viewportWidth * viewportScale;

viewport.style.width = viewportWidth + "px";
viewport.style.height = viewportHeight + "px";

const context = viewport.getContext("2d");

context.scale(viewportScale, viewportScale);

context.fillStyle = "rgb(0, 0, 0)";

const simulation = new sim.Simulation();

function redraw() {
	context.clearRect(0, 0, viewportWidth, viewportHeight);

	const world = simulation.world();

	simulation.step();

	for (const food of world.foods) {
		context.drawCircle(
			food.x * viewportWidth,
			food.y * viewportHeight,
			(0.01 / 2.0) * viewportWidth,
		);
	}

	for (const animal of world.animals) {
		context.drawTriangle(
			animal.x * viewportWidth,
			animal.y * viewportHeight,
			0.02 * viewportWidth,
			animal.rotation,
		);
	}

	requestAnimationFrame(redraw);
}

redraw();
