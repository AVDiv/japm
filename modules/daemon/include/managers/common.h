#include <csignal>
#include <cstdint>
#include <ctime>
#include <expected>
#include <optional>
#include <string>

enum ChildProcessStatus { Pending, Sleeping, Running, Exited };

struct ChildProcess {
  std::string command;
  uint32_t pid;
  std::string user;
  time_t spawn_time;
  ChildProcessStatus status;
  std::optional<std::sig_atomic_t> exit_code;
};

class ProcessManager {
public:
  // Constructors & Destructors
  virtual ~ProcessManager() = default;
  // Getters
  virtual uint16_t get_children() = 0;
  virtual std::optional<ChildProcess> get_child_by_pid(uint32_t pid) = 0;
  virtual time_t get_manager_spawn_time() = 0;
  // Other Functions
  std::expected<std::string, std::string>
  spawn_process(std::string command) = 0;
  virtual void record_process_status(uint32_t pid) = 0;
};
