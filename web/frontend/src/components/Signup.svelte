<script lang="ts">
    import SwapTextButton from "../util/SwapTextButton.svelte";
    import SwapTextLink from "../util/SwapTextLink.svelte";

    let email = "";
    let password = "";
    let username = "";
    let errors: string[] = [];

    async function auth() {
        errors = [];
        let response = await fetch("/api/site/signup", {
            method: "POST",
            mode: "same-origin",
            body: JSON.stringify({ email, password, username }),
            headers: { "Content-Type": "application/json" },
        });

        if (!response.ok) {
            let json: any = await response.json();

            if (json.already_exists === "email") {
                errors = [...errors, "That email is already registered."];
            }

            if (json.already_exists === "uname") {
                errors = [
                    ...errors,
                    "That username is already registered. Maybe try a different one?",
                ];
            }
        } else {
            // log in on successful auth
            window.location.replace("/");
        }
    }
</script>

<div class="form">
    <h1>Let's get you in!</h1>
    <form>
        {#if errors.length != 0}
            <ul id="errors">
                {#each errors as error}
                    <li>{error}</li>
                {/each}
            </ul>
        {/if}
        <label for="username">Username</label>
        <input
            id="username"
            class="input-field"
            type="text"
            required
            name="username"
            bind:value={username}
            placeholder="Squidbeak" />
        <label for="email">Email</label>
        <input
            id="email"
            class="input-field"
            type="email"
            required
            name="email"
            bind:value={email}
            placeholder="user@example.com" /><br />
        <label for="password">Password</label>
        <input
            id="password"
            class="input-field"
            bind:value={password}
            type="password"
            placeholder="doNotUseThisPassword" /><br />
        <SwapTextButton
            initial_text="Sign Up"
            swap_text="You're in for something special."
            callback={async () => await auth()} />
        <br />
        <SwapTextLink
            dest="/#/signin"
            swap_text="Let's get you to the right place."
            initial_text="Already have an account?" />
    </form>
</div>

<style lang="scss">
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
    }

    #errors {
        border: 1px solid #dd0000;
        border-radius: 7px;
        padding: 7px;
        list-style-type: none;
    }
</style>
