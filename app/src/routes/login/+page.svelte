<script lang="ts">
  import '../styles.css';
  import { api } from '../../api';
  let username = '';
  let password = '';
  const login = () =>{
    api.login(username, password);
  }
</script>

<h1>登入頁面</h1>

<div class="text-center">
  <div class="text-2xl py-2">
    帳號
    <input
      class="w-40 border-2 border-blue-500 rounded-md h-10 text-center text-2xl"
      bind:value={username}
    />
  </div>
  <div class="text-2xl py-2">
    密碼
    <input
      class="w-40 border-2 border-blue-500 rounded-md h-10 text-center text-2xl"
      type="password"
      bind:value={password}
    />
  </div>
  <div class="py-2" >
    <button
      class="w-40 bg-amber-400 border-2 border-blue-500 rounded-md h-10 text-center text-2xl p"
      on:click={login}
    >
      登入
    </button>
  </div>
</div>
