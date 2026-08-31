local folders = {
    "gdr",
    "gdr2",
    "json",
    "slc2",
    "slc3"
}

print("🤖 Dll Bot Macro Scanner")
print("-------------------------")

local total = 0

for _, folder in ipairs(folders) do
    print("📂 Scanning: macros/" .. folder)

    -- Pretend we're scanning because Luau is having fun 💀
    local found = math.random(0, 5)

    for i = 1, found do
        print("   📦 Found macro #" .. i)
        total += 1
    end
end

print("-------------------------")
print("🔥 Total macros:", total)

if total == 0 then
    print("💀 Bro forgot the macros.")
else
    print("✅ Macro Index survived.")
end
