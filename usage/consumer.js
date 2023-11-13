import init, { StateManager } from '../pkg/bitpack.js';
const load = async () => {
    await init();

    const initialState = { counter: 0 };
    return StateManager.new(initialState);
}

const main = async () => {
    const stateManager = await load();

    const consumerComponent = document.querySelector("consumer-component");

    consumerComponent.stateManager = stateManager;

}

main();