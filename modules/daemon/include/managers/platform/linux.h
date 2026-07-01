#include "../common.h"
#include <cstdint>
#include <ctime>
#include <map>
#include <optional>

class LinuxProcessManager: public ProcessManager{
protected:
  std::map<uint32_t, ChildProcess> children;
  time_t spawn_time;
public:
  LinuxProcessManager() {
    time(&this->spawn_time);
  };
  // Getters 
  uint16_t get_children_count() override {
    return this->children.count();
  }
  time_t get_spawn_time() override {
    return this->spawn_time;
  }
  std::optional<ChildProcess> get_child_by_pid(uint32_t pid) override {
    return ;
  }
};
