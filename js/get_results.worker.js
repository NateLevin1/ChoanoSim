self.onmessage = ({
    reproMethod,
    beginningFoodDensity,
    switchedFoodDensity,
}) => {
    // run Rust main
    import("../pkg/index.js").catch(console.error).then(init);

    function init(rustModule) {
        console.log("Getting results...");
        const results = rustModule.get_results_csv(
            reproMethod,
            beginningFoodDensity,
            switchedFoodDensity
        );
        console.log("Got results.");
        self.postMessage({ type: "finished", results });
    }

    // this is called from rust via eval :^)
    self.onCompletionPercentChange = (percent) => {
        self.postMessage({ type: "update-percent", percent });
    };
};
