<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import {window} from "@tauri-apps/api";

    let time;
    time = "00:00:00";
    let window_the_pin = false;
    async function Clock()
    {
        setInterval(async function () {
            time = await invoke("get_now_time").then(value => {
                return value;
            });
        }, 500);
    }

    async function SetWind() {
        window_the_pin = !window_the_pin;
        window.getCurrent().setAlwaysOnTop(window_the_pin).then(r =>{ return r;});
    }

    onload = async function onload() {
        await Clock();
    }
</script>

<div>
    <h2>{time}</h2>
    <button class="button-1" role="button" on:click={SetWind}>{window_the_pin ? "Unpin" : "Pin"}</button>
</div>

<style>
    h2 {
        color: yellow;
        font-family: ‘Helvetica Neue’, sans-serif;
        font-size: 25px;
        font-weight: bold;
        letter-spacing: -1px;
        line-height: 1;
        text-align: center;
        text-shadow: #0f0f0f;
        filter: drop-shadow(0 0 2em #ffa427);
    }

    .button-1 {
        background-color: #ffb626;
        border-radius: 8px;
        border-style: none;
        box-sizing: border-box;
        color: #FFFFFF;
        cursor: pointer;
        display: inline-block;
        font-family: "Haas Grot Text R Web", "Helvetica Neue", Helvetica, Arial, sans-serif;
        font-size: 14px;
        font-weight: 500;
        height: 40px;
        line-height: 20px;
        list-style: none;
        margin: 0;
        outline: none;
        padding: 10px 16px;
        position: relative;
        text-align: center;
        text-decoration: none;
        transition: color 100ms;
        vertical-align: baseline;
        user-select: none;
        -webkit-user-select: none;
        touch-action: manipulation;
    }
</style>