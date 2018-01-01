PROJECT(GeometryAlgorithmLibrary)
cmake_minimum_required(VERSION 3.0)

## 実行ファイルの出力ディレクトリの設定
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/bin)

## ライブラリファイルの出力ディレクトリの設定
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR}/lib)
link_directories(${CMAKE_SOURCE_DIR}/lib)

## インクルードディレクトリの設定
include_directories(${CMAKE_SOURCE_DIR}/include)

## プロジェクトに含むソースコードのトラッキング
FILE(GLOB_RECURSE ALL_FILES
	RELATIVE ${CMAKE_SOURCE_DIR}
 	include/*.hpp src/*.cpp)

## ディレクトリ構成を保持したままビルドシステムを構築するマクロ
foreach(FILE ${ALL_FILES})
  get_filename_component(PARENT_DIR "${FILE}" PATH)
  # skip src or include and changes /'s to \\'s
  string(REGEX REPLACE "(\\./)?(src|include)/?" "" GROUP "${PARENT_DIR}")
  string(REPLACE "/" "\\" GROUP "${GROUP}")
  source_group("${GROUP}" FILES "${FILE}")
endforeach()


# C++11を有効にする
add_definitions("-Wall -std=c++11")

add_library(gal STATIC
	${ALL_FILES}
)

add_subdirectory(sample)