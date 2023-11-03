<script lang="ts">
    import PrismarineCoLogo from "./assets/pc_gear_v3.svg";
    import Login from "./components/Login.svelte";
    import Signup from "./components/Signup.svelte";
    import Router from "svelte-spa-router";
    import { slide } from "svelte/transition";

    let sidebar = document.getElementById("sidebar")!;
    let sidebar_open: boolean = false;
    const toggle_sidebar = () => {
        sidebar_open = !sidebar_open;
    };

    const routes = {
        "/signin": Login,
        "/signup": Signup,
    };
</script>

<div id="toplevel">
    <nav>
        <img
            id="header-logo"
            src={PrismarineCoLogo}
            alt="The PrismarineCo logo: a hexagon surrounding a gear" />
        <p id="header-company">
            <span id="header-company-name">PrismarineCo.</span>
            <span id="header-company-name-long">The Prismarine Company</span>
        </p>
        <button on:click|preventDefault={toggle_sidebar}>
            <svg id="navclick-svg" viewBox="0 0 100 80" width="40" height="40">
                <rect y="5" width="100" height="10"></rect>
                <rect y="35" width="100" height="10"></rect>
                <rect y="65" width="100" height="10"></rect>
            </svg>
        </button>
    </nav>
    {#if sidebar_open}
        <aside id="sidebar" transition:slide>
            <ul>
                <li>item 1</li>
                <li>item 2</li>
            </ul>
        </aside>
    {/if}
    <main>
        <div class="noshow">
            <Router {routes} />
        </div>
    </main>
</div>

<style>
    .noshow {
        visible: false;
    }
    #toplevel {
        display: grid;
        grid-template-columns: 4fr 1fr;
    }

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

    main,
    nav {
        transition: margin-right 0.5;
    }

    nav {
        grid-area: nav;
        grid-row: 1 / 2;
        display: flex;
        justify-content: space-between;
        border-bottom: 1px solid #ffffff;
    }

    #navclick-svg {
        fill: white;
    }

    #sidebar {
        grid-column: 2;
        grid-row-start: 2;
        border-left: 1px solid #fff;
        background-color: #0f0f0f;
        overflow-x: hidden;
        transition: 0.5;
    }

    main {
        grid-row: 2/3;
        grid-column: 1/2;
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
