#include "atari_ntsc_rgb_palette.h"

class ALEInterface;
class ALEState;

ALEInterface *ALE_new();
void ALE_del(ALEInterface *ale);
const char *getString(ALEInterface *ale, const char *key);
int getInt(ALEInterface *ale, const char *key);
bool getBool(ALEInterface *ale, const char *key);
float getFloat(ALEInterface *ale, const char *key);
void setString(ALEInterface *ale, const char *key, const char *value);
void setInt(ALEInterface *ale, const char *key, int value);
void setBool(ALEInterface *ale, const char *key, bool value);
void setFloat(ALEInterface *ale, const char *key, float value);
void loadROM(ALEInterface *ale, const char *rom_file);
int act(ALEInterface *ale, int action);
bool game_over(ALEInterface *ale);
void reset_game(ALEInterface *ale);
void getAvailableModes(ALEInterface *ale, int *availableModes);
int getAvailableModesSize(ALEInterface *ale);
void setMode(ALEInterface *ale, int mode);
void getAvailableDifficulties(ALEInterface *ale, int *availableDifficulties);
int getAvailableDifficultiesSize(ALEInterface *ale);
void setDifficulty(ALEInterface *ale, int difficulty);
void getLegalActionSet(ALEInterface *ale, int *actions);
int getLegalActionSize(ALEInterface *ale);
void getMinimalActionSet(ALEInterface *ale, int *actions);
int getMinimalActionSize(ALEInterface *ale);
int getFrameNumber(ALEInterface *ale);
int lives(ALEInterface *ale);
int getEpisodeFrameNumber(ALEInterface *ale);
void getScreen(ALEInterface *ale, unsigned char *screen_data);
void getRAM(ALEInterface *ale, unsigned char *ram);
int getRAMSize(ALEInterface *ale);
int getScreenWidth(ALEInterface *ale);
int getScreenHeight(ALEInterface *ale);
void getScreenRGB(ALEInterface *ale, unsigned char *output_buffer);
void getScreenRGB2(ALEInterface *ale, unsigned char *output_buffer);
void getScreenGrayscale(ALEInterface *ale, unsigned char *output_buffer);
void saveState(ALEInterface *ale);
void loadState(ALEInterface *ale);
ALEState *cloneState(ALEInterface *ale);
void restoreState(ALEInterface *ale, ALEState *state);
ALEState *cloneSystemState(ALEInterface *ale);
void restoreSystemState(ALEInterface *ale, ALEState *state);
void deleteState(ALEState *state);
void saveScreenPNG(ALEInterface *ale, const char *filename);
void encodeState(ALEState *state, char *buf, int buf_len);
int encodeStateLen(ALEState *state);
ALEState *decodeState(const char *serialized, int len);
void setLoggerMode(int mode);
