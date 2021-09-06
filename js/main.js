log("Hello from JavaScript");
log(`log = ${log}`);
log(`peripherals = ${peripherals}`);
log(`getIds()`);
log(typeof peripherals.getIds());
log(`Is array: ${Array.isArray(peripherals.getIds())}`)
log(peripherals.getIds().length);

function displayPeripherals() {
    let ids = peripherals.getIds();

    log(`Available peripherals:`);
    log(`----------------------`);

    for (let id of ids) {
        let type = peripherals.getType(id);
        log(`• ID: ${id}, Type: ${type}`);
    }
}

displayPeripherals();

const a = peripherals.get('ARITHMETIC');
log(`Sum result: ${a.sum(5, 5, 7, 0.5)}`);

// log('Pulling an event');
// let promise = peripherals.pullEvent();
// log(`promise: ${promise}`);

function displayEvent(event) {
    log(
`Event: (${event}) {
    peripheralId: ${event.peripheralId}
    name: ${event.name}
    data: ${event.data}
}`
    );
}

async function main() {
    log(`Pulling first event`);
    const ev1 = await peripherals.pullEvent();
    displayEvent(ev1);

    log(`Pulling second event passively`);
    const ev2 = await peripherals.pullEvent(true);
    log(`ev2: ${ev2}`);

    log(`Pulling an event aggressively`);
    const ev3 = await peripherals.pullEvent();
    displayEvent(ev3);

    log('Finished pulling');
}

main().catch(() => {
    log(`MAIN FAILED!`);
});

//let peripherals = core.getPeripheralsIds();
//log(typeof peripherals);
//log(`Length: ${peripherals.length}`)
//log(core.getPeripheral("1"));
//log(core.getPeripheralType("2"));
//log(new Function("return 5;")());