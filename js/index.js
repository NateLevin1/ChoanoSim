// run Rust main
import("../pkg/index.js").catch(console.error).then(init);

let isPlaying = false;
let steps = 0;
let rustSimulator = null;
let context;
const simWidth = 800;
let camera = {
    x: window.innerWidth / 2 - simWidth / 2,
    y: 20,
    zoom: 1,
};

function runStep(rustModule) {
    updateTime();
    rustModule.simulate_step(rustSimulator);
    rustModule.render_simulator(context, camera.x, camera.y);
    if (isPlaying) {
        requestAnimationFrame(() => runStep(rustModule));
    }
}

function updateTime() {
    steps++;
    document.getElementById("time").textContent = `${steps} step${
        steps > 1 ? "s" : ""
    }`;
}

function init(rustModule) {
    let canvas = document.getElementById("canvas");
    context = canvas.getContext("2d");
    rustModule.initialize_canvas(context);
    console.log("Init complete.");
    rustModule.render_simulator(context, camera.x, camera.y);

    const controls = document.getElementById("controls");
    const step = document.getElementById("step");
    const play = document.getElementById("play");
    const genes = document.getElementById("genes");
    const inspector = document.getElementById("inspector");
    const reproRadios = Array.from(
        document.querySelectorAll("input[name=reproduction]")
    );
    const foodDensity = document.getElementById("food-density");

    // play/step
    step.onclick = () => {
        if (isPlaying) return;
        controls.classList.add("playing");
        setTimeout(() => {
            controls.classList.remove("playing");
        }, 50);
        runStep(rustModule);
    };
    play.onclick = () => {
        isPlaying = !isPlaying;
        play.textContent = isPlaying ? "Pause" : "Play";
        controls.classList.toggle("playing");
        if (isPlaying) {
            runStep(rustModule);
        }
    };

    // conf
    foodDensity.onchange = (event) => {
        const newVal = parseInt(event.target.value);
        rustModule.set_food_density(newVal);
    };
    reproRadios.forEach((el) => {
        el.onchange = (event) => {
            const newRepro = event.target.id;
            rustModule.set_reproductive_method(newRepro);
        };
    });

    // renderer
    window.onkeydown = (event) => {
        switch (event.key) {
            case "ArrowUp":
            case "w":
                camera.y -= 100;
                break;
            case "ArrowDown":
            case "s":
                camera.y += 100;
                break;
            case "ArrowRight":
            case "d":
                camera.x += 100;
                break;
            case "ArrowLeft":
            case "a":
                camera.x -= 100;
        }
        rustModule.render_simulator(context, camera.x, camera.y);
    };
    window.onwheel = (event) => {
        camera.x -= event.deltaX;
        camera.y -= event.deltaY;
        rustModule.render_simulator(context, camera.x, camera.y);
    };
}
