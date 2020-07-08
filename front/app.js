run = async() => {
    const { run } = await import('../pkg')
    new run()
};

run();