<script lang="ts">
    import SwapTextButton from "../../util/SwapTextButton.svelte";
    import SwapTextLink from "../../util/SwapTextLink.svelte";
    let email = "";
    let password = "";
    let errors: string[] = [];

    async function auth() {
        errors = [];
        let response = await fetch("/api/site/signin", {
            method: "POST",
            mode: "same-origin",
            body: JSON.stringify({ email, password }),
            headers: { "Content-Type": "application/json" },
        });

        if (!response.ok) {
            if (response.status === 404) {
                errors = [...errors, "No account exists with that email."];
            } else if (response.status === 401) {
                errors = [...errors, "The password is incorrect."];
            }
        } else {
            // log in on successful auth
            window.location.replace("/");
        }
    }
</script>

<div class="form">
    <form>
        {#if errors.length != 0}
            <ul id="errors">
                {#each errors as error}
                    <li>{error}</li>
                {/each}
            </ul>
        {/if}
        <label for="email">Email</label>
        <input
            id="email"
            class="input-field"
            type="email"
            required
            name="email"
            autocomplete="username"
            bind:value={email}
            placeholder="user@example.com" /><br />
        <label for="password">Password</label>
        <input
            id="password"
            class="input-field"
            bind:value={password}
            type="password"
            placeholder="doNotUseThisPassword" />
        <SwapTextButton
            initial_text="Sign In"
            swap_text="Welcome back."
            callback={async () => await auth()} />
        <SwapTextLink
            dest="/#/signup"
            initial_text="Don't have an account?"
            swap_text="Let's fix that." />
    </form>
</div>

<style>
    * {
        box-sizing: border-box;
    }

    div.form {
        display: block;
        text-align: center;
    }

    .input-field {
        padding: 5px;
        margin: 5px;
    }

    form {
        display: flex;
        align-items: center;
        flex-direction: column;
        box-sizing: border-box;
    }

    #errors {
        border: 1px solid #dd0000;
        border-radius: 7px;
        padding: 7px;
        list-style-type: none;
    }
</style>
