import { invoke } from "@tauri-apps/api/tauri";

function Tasks() {
  return (
    <div style={{padding:40}}>
      任务列表
      <button
        onClick={() => {
          invoke("start_aria2c", {arg1: "arg1"});
        }}
      >
        启动
      </button>
    </div>
  );
}

export default Tasks;
