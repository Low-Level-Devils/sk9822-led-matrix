function send_animation_request() {
    fetch('/request', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ animation: options.selectedOptions[0].text }),
    });
}

async function loadAnimations() {
    try {
        const resp = await fetch('animations.json');
        const data = await resp.json();

        for (let key in data) {
            const opt = new Option(key);
            options.add(opt);
        }
    } catch (error) {
        console.error('Error:', error);
    }
}

loadAnimations();

const butt1 = document.getElementById('butt1');
const options = document.getElementById('options');

butt1.addEventListener('click', send_animation_request);