# lunar-python ↔ lunar_rust 映射与同步基线

## 上游基线（本次对齐）
- 上游仓库：`https://github.com/6tail/lunar-python`
- 本地对齐源：`/Volumes/data/code/lunar/lunar-python`
- 对齐基线 commit：`448f397c1695cadab3899bf460e0042cab7f0e66`
- 记录日期：`2026-02-10`

## 模块映射
- `lunar_python/Solar.py` → `src/lib/solar.rs`, `src/lib/solar/helper_traits.rs`
- `lunar_python/Lunar.py` → `src/lib/lunar.rs`, `src/lib/lunar/helper_traits.rs`
- `lunar_python/LunarYear.py` → `src/lib/lunar_year.rs`
- `lunar_python/LunarMonth.py` → `src/lib/lunar_month.rs`
- `lunar_python/LunarTime.py` → `src/lib/lunar_time.rs`, `src/lib/lunar_time/helper_triats.rs`
- `lunar_python/SolarWeek.py` → `src/lib/solar_week.rs`, `src/lib/solar_week/helper_traits.rs`
- `lunar_python/SolarMonth.py` → `src/lib/solar_month.rs`, `src/lib/solar_month/helper_traits.rs`
- `lunar_python/SolarSeason.py` → `src/lib/solar_season.rs`
- `lunar_python/SolarHalfYear.py` → `src/lib/solar_half_year.rs`（本次新增）
- `lunar_python/SolarYear.py` → `src/lib/solar_year.rs`
- `lunar_python/EightChar.py` → `src/lib/eight_char.rs`
- `lunar_python/Tao.py` → `src/lib/tao.rs`, `src/lib/tao/helper_traits.rs`
- `lunar_python/Foto.py` → `src/lib/foto.rs`, `src/lib/foto/helper_traits.rs`
- `lunar_python/Holiday.py` → `src/lib/holiday.rs`
- `lunar_python/NineStar.py` → `src/lib/nine_star.rs`
- `lunar_python/JieQi.py` → `src/lib/jie_qi.rs`
- `lunar_python/Fu.py` → `src/lib/fu.rs`
- `lunar_python/ShuJiu.py` → `src/lib/shu_jiu.rs`
- `lunar_python/TaoFestival.py` → `src/lib/tao_festival.rs`
- `lunar_python/FotoFestival.py` → `src/lib/foto_festival.rs`

## 命名差异兼容（本次补齐）
- Solar：`from_ymd_hms`/`from_ymdhms`，`from_ba_zi`/`from_bazi`，`to_ymd_hms`/`to_ymdhms`
- LunarYear：`from_year`/`from_lunar_year`，`compute`，`get_leap_month`，`get_tou_liang`，`get_cao_zi`
- Foto：`from_ymd`

## 快速再对齐流程（上游更新后）
1. 更新上游并记录新基线：
   - `git -C /Volumes/data/code/lunar/lunar-python fetch --all --tags`
   - `git -C /Volumes/data/code/lunar/lunar-python rev-parse HEAD`
2. 对照本文件“模块映射”逐模块比对方法签名与行为。
3. 优先补齐新增 API/别名，再修复行为差异。
4. 执行验证：`cargo test -- --nocapture`。
5. 更新本文件中的“对齐基线 commit”和日期。
