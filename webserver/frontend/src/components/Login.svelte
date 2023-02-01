<script lang="ts">
    import { getCookie, removeCookie } from "typescript-cookie";
    import { current_user } from "../lib/userstore";

    let cookie = getCookie("discord_user");
    if (cookie !== undefined) {
        $current_user = JSON.parse(cookie);
    }

    function on_logout() {
        removeCookie("discord_user");
        $current_user = null;
        window.location.href = "/";
    }

    function on_login() {
        window.location.href = "/login";
    }
</script>

{@debug current_user}

<div id="login">
    {#if $current_user !== null}
        Hello, {$current_user.username}.
        <button on:click={on_logout}>Logout</button>
    {:else}
        <button on:click={on_login}>Login</button>
    {/if}
</div>

<style>
 #login {
   float: left;
 }
</style>
