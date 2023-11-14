class ConsumerComponent extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

    set stateManager(manager){
        this._stateManager = manager;
        this.updateState();
    }

    connectedCallback() {
        this.render();
        this.addEventListener();
    }

    render(){
        this.shadowRoot.innerHTML = `
            <div>
                <span id="counter-value">0</span>
                <button id="increment-btn">Increment</button>
            </div>
        `
    }

    addEventListener() {
        this.shadowRoot.querySelector('#increment-btn').addEventListener('click', () => {
           let currentState = this._stateManager.get_state();
           currentState.counter += 1;
           this._stateManager.update_state(currentState);
           this.updateState();
        });
    }

    updateState(){
        if(!this._stateManager) return;
        const state = this._stateManager.get_state();
        this.shadowRoot.querySelector("#counter-value").textContent = state.counter;
    }
}

customElements.define("consumer-component", ConsumerComponent);