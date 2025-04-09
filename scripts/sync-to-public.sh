#!/bin/bash

# 설정
PRIVATE_REPO_DIR=$(pwd)
PUBLIC_REPO_NAME="LoaAssist"
PUBLIC_REPO_DIR="../loaassist-public"
TEMP_DIR="../loaassist-temp"
PUBLIC_REMOTE="git@github.com:CasinGit/LoaAssist.git"

echo "🔁 비공개(main) → 공개(main) 일반 동기화 시작..."

# 클린업
rm -rf "$TEMP_DIR"
mkdir "$TEMP_DIR"

# 비공개 레포의 main 브랜치 내용 복사
git --work-tree="$TEMP_DIR" checkout main -- .

# 공개용 README 교체
cp "$PRIVATE_REPO_DIR/README.public.md" "$TEMP_DIR/README.md"
rm "$TEMP_DIR/README.public.md"

# 공개 레포가 없다면 clone
if [ ! -d "$PUBLIC_REPO_DIR" ]; then
  git clone "$PUBLIC_REMOTE" "$PUBLIC_REPO_DIR"
fi

cd "$PUBLIC_REPO_DIR"

# 변경된 파일만 갱신
git pull origin main
cp -r "$TEMP_DIR"/* .

git add .
git commit -m "Update from LoaAssist_dev"
git push origin main

echo "✅ 공개 레포 '$PUBLIC_REPO_NAME'로의 일반 동기화 완료"
