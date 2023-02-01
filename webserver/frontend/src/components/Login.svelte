<script lang="ts">
    import { getCookie, removeCookie } from "typescript-cookie";

    export let current_user = null;

    let cookie = getCookie("discord_user");
    if (cookie !== undefined) {
        current_user = JSON.parse(cookie);
    }

    function on_logout() {
        removeCookie("discord_user");
        current_user = null;
    }

    function on_login() {
        window.location.href = "/login";
    }
</script>

<main>
    {#if current_user !== null}
        Hello, {current_user.username}.
        <button on:click={on_logout}>Logout</button>
    {:else}
        <button on:click={on_login}>Login</button>
    {/if}
</main>
