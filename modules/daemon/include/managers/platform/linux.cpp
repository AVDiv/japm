#include "../common.cpp"
#include <csignal>
#include <cstdint>
#include <ctime>
#include <iostream>
#include <map>
#include <optional>
#include <unistd.h>

class LinuxProcessManager : public ProcessManager {
protected:
  std::map<uint32_t, ChildProcess> children;
  time_t spawn_time;

public:
  LinuxProcessManager() {
    time(&this->spawn_time);
    signal(SIGCHLD, SIG_IGN);
  };
  // Getters
  uint16_t get_children_count() { return this->children.size(); }
  time_t get_spawn_time() { return this->spawn_time; }
  std::optional<ChildProcess *> get_child_by_pid(pid_t pid) {
    return &this->children[pid];
  }
  // Other Functions
  pid_t *spawn_process(std::string command) {
    if (signal(SIGCHLD, SIG_IGN) == SIG_ERR) {
      std::cerr << "signal";
      exit(EXIT_FAILURE);
    }
    // Fork the process
    pid_t pid = fork();
    if (pid == 0) {
      // For the child process (Spawn the new process)
      const char *comm_c_str = command.c_str();
      execl(comm_c_str, comm_c_str, NULL);
      // Set Child process structure
      this->children[pid] = ChildProcess();
      this->children[pid].command = std::move(command);
      this->children[pid].user = nullptr;
      this->children[pid].pid =
          std::move(pid); // PID moved to child process, can't use `pid`
                          // local_var to access map again

      return &pid;

    } else if (pid > 0) {
      // For the parent process
      std::cout << "The parent process implementation";
      return &pid;
    } else {
      std::cerr << "Unknown failure occured while forking the process";
      return &pid;
    }
  }

  void record_process_status(pid_t pid) {}

private:
  void spawn_manager_cycle(pid_t pid) {}
};
