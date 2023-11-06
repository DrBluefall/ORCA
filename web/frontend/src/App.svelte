<script lang="ts">
    import PrismarineCoLogo from "./assets/pc_gear_v3.svg";
    import Login from "./components/pages/Login.svelte";
    import Signup from "./components/pages/Signup.svelte";
    import Index from "./components/pages/Index.svelte";
    import Router from "svelte-spa-router";
    import { link } from "svelte-spa-router";
    import { slide } from "svelte/transition";

    let sidebar_open: boolean = false;
    const toggle_sidebar = () => {
        sidebar_open = !sidebar_open;
    };

    const routes = {
        "/": Index,
        "/signin": Login,
        "/signup": Signup,
    };
</script>

<div id="toplevel">
    <div id="site-header">
        <a href="/" use:link>
            <img
                id="header-logo"
                src={PrismarineCoLogo}
                alt="The PrismarineCo logo: a hexagon surrounding a gear" />
        </a>
        <a href="/" use:link id="root-link">
            <p id="header-company">
                <span id="header-company-name">PrismarineCo.</span>
                <span id="header-company-name-long"
                    >The Prismarine Company</span>
            </p>
        </a>
        <button on:click|preventDefault={toggle_sidebar}>
            <svg id="navclick-svg" viewBox="0 0 100 80" width="40" height="40">
                <rect y="5" width="100" height="10"></rect>
                <rect y="35" width="100" height="10"></rect>
                <rect y="65" width="100" height="10"></rect>
            </svg>
        </button>
    </div>
    <div id="site-body">
        {#if sidebar_open}
            <nav
                id="sidebar"
                class="is-visible"
                transition:slide={{ axis: "x" }}>
                <ul id="navlinks">
                    <li><a href="/signin" use:link>Sign In</a></li>
                </ul>
            </nav>
        {/if}
        <main>
            <Router {routes} />
        </main>
    </div>
</div>

<style>
    button {
        border: none;
        background-color: transparent;
        font-family: inherit;
        padding: 0.9%;
        cursor: pointer;

        @media screen and (-ms-high-contrast: active) {
            border: 2px solid currentcolor;
        }
    }
    #root-link {
        text-decoration: none;
        background-color: inherit;
        color: inherit;
    }

    main,
    #site-header {
        transition: margin-right 0.5;
    }

    #site-header {
        width: 100%;
        display: flex;
        justify-content: space-between;
        border-bottom: 1px solid #ffffff;
    }

    #navclick-svg {
        fill: white;
    }

    #site-body {
        display: flex;
        justify-content: flex-end;
        flex-direction: row-reverse;
    }

    main {
        padding: 0.5rem 0.5rem;
        flex: 1 0 100%;
    }

    #sidebar {
        height: 100%;
        flex: 0 0 auto;
        border-left: 1px solid #fff;
        background-color: #0f0f0f;
        overflow-x: hidden;
        transition: 0.5;
    }
    #sidebar.is-visible ~ main {
        flex-basis: 0;
    }

    #navlinks {
        padding: 1rem;
    }

    #header-logo {
        height: 2.25rem;
        padding: 20px;
    }

    #header-company {
        font-size: 2rem;
        padding: 20px;
    }

    @media (width > 700px) or (width < 440px) {
        #header-company-name {
            display: none;
        }
    }

    @media (width <= 700px) {
        #header-company-name-long {
            display: none;
        }
        #sidebar {
            width: 100%;
        }
    }
</style>
