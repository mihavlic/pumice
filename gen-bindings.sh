#!/usr/bin/sh

VULKAN_DOCS="https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/main"

if ! test -f "vk.xml"; then
    echo Downloading vk.xml
    wget "$VULKAN_DOCS/xml/vk.xml" 2> /dev/null
fi

if ! test -f "video.xml"; then
    echo Downloading video.xml
    wget "$VULKAN_DOCS/xml/video.xml" 2> /dev/null
fi

if test -z "$1"; then
    echo \
'Missing bindings output path, example usage:
  ./gen-bindings.sh $OUT_PATH "VK_VERSION_1_0, @surface, VK_KHR_SWAPCHAIN"'
    exit 1
fi

if test -z "$2"; then
    echo \
'Missing generated extensions / features selection, example usage:
  ./gen-bindings.sh $OUT_PATH "VK_VERSION_1_0, @surface, VK_KHR_SWAPCHAIN"'
    exit 1
fi

if command -v mold &> /dev/null; then
    mold -run cargo run --bin generate $(realpath vk.xml)  $(realpath video.xml) ./template "$1" "$2"
else 
    cargo run --bin generate $(realpath vk.xml)  $(realpath video.xml) ./template "$1" "$2"
fi