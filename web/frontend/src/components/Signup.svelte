<script lang="ts">
    let email = "";
    let password = "";
    let username = "";
    let errors: string[] = [];
    let login_redirect: string = "Already have an account?";
    let signup_text: string = "Sign Up";

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
                errors = [...errors, "That username is already registered. Maybe try a different one?"];
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
            placeholder="Squidbeak"
        />
        <label for="email">Email</label>
        <input
            id="email"
            class="input-field"
            type="email"
            required
            name="email"
            bind:value={email}
            placeholder="user@example.com"
        /><br />
        <label for="password">Password</label>
        <input
            id="password"
            class="input-field"
            bind:value={password}
            type="password"
            placeholder="doNotUseThisPassword"
        /><br />
        <button
            class="button"
            on:click|preventDefault={async () => await auth()}
            on:mouseenter={() => {
                            signup_text = "You're in for something special.";
                          }}
            on:mouseleave={() => {
                            signup_text = "Sign Up";
                         }}>{signup_text}</button>
        <br />
        <a href="/#/signin"
           class="button"
           on:mouseenter={() => {
                            login_redirect = "Let's get you to the right place.";
                         }}
           on:mouseleave={() => {
                            login_redirect = "Already have an account?";
                         }}>{login_redirect}</a>
    </form>
</div>

<style lang="scss">
    $backgroundColor: #000000;
    $color: #FFFFFF;

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

    a, button {
        font-size: 1.15rem;
    }

    a.button {
        text-decoration: none;
    }

    button.button {
        border: none;
        background-color: transparent;
        font-family: inherit;
        padding: 0;
        cursor: pointer;

        @media screen and (-ms-high-contrast: active) {
            border: 2px solid currentcolor;
        }
    }

    a.button,
    button.button {
        // Display
        display: inline-flex;
        align-items: center;
        justify-content: center;

        // Visual
        background-color: $backgroundColor;
        color: $color;
        border-radius: 8px;
        box-shadow: 0 3px 5px rgba(0, 0, 0, 0.18);

        // Size
        padding: 0.25em 0.75em;
        min-width: 10ch;
        min-height: 44px;

        // Text
        text-align: center;
        line-height: 1.1;

        // Spacing
        margin-top: 5px;
        margin-bottom: 5px;

        transition: 220ms all ease-in-out;

        &:hover,
        &:active {
            background-color: $color;
            color: $backgroundColor;
        }
    }

    #errors {
        border: 1px solid #DD0000;
        border-radius: 7px;
        padding: 7px;
        list-style-type: none;
    }

</style>
