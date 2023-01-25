#include <gtest/gtest.h>
#include <gmock/gmock.h>
#include "../src/lib/IpcCopyFile.h"
#include "Gtest_Ipc.h"
#include <deque>
#include <vector>
#include <sstream>


using ::testing::Eq;
using ::testing::StrEq;
using ::testing::IsTrue;
using ::testing::IsFalse;
toolBox myToolBox;

TEST(TestSimpleMethods, GetBufferSize)
{
    FileManipulationClassReader dummyCopyFileReader(&myToolBox);
    EXPECT_THAT(dummyCopyFileReader.getBufferSize(),Eq(dummyCopyFileReader.getDefaultBufferSize()));

    FileManipulationClassWriter dummyCopyFileWriter(&myToolBox);
    EXPECT_THAT(dummyCopyFileWriter.getBufferSize(),Eq(dummyCopyFileWriter.getDefaultBufferSize()));
}

TEST(FileManipulation, OpenFile)
{
    
    FileManipulationClassReader dummyCopyFileReader(&myToolBox);
    FileManipulationClassWriter dummyCopyFileWriter(&myToolBox);
    {
        std::string dummyFile = "IamTestingToOpenThisFile";
        EXPECT_THROW(dummyCopyFileReader.openFile(dummyFile.c_str()),file_exception);
    }

    std::string fileToOpenForWriting = "fileToOpenForWriting";
    EXPECT_NO_THROW(dummyCopyFileWriter.openFile(fileToOpenForWriting.c_str()));
    EXPECT_NO_THROW(dummyCopyFileReader.openFile(fileToOpenForWriting.c_str()));

    
    {
        FileManipulationClassWriter openAnotherTime(&myToolBox);
        CaptureStream stdcout{std::cout};
        EXPECT_NO_THROW(openAnotherTime.openFile(fileToOpenForWriting.c_str()));
        EXPECT_THAT(stdcout.str(), StrEq("The file specified to write in already exists. Data will be erased before proceeding.\n"));
    }

    remove(fileToOpenForWriting.c_str());

    {
        
        CreateRandomFile randomFile {"testfile.dat", 10, 1};
        FileManipulationClassReader openRandomFile(&myToolBox);
        EXPECT_NO_THROW(openRandomFile.openFile(randomFile.getFileName()));
        {
            FileManipulationClassWriter openRandomFileWriter(&myToolBox);
            CaptureStream stdcout{std::cout};
            EXPECT_NO_THROW(openRandomFileWriter.openFile(randomFile.getFileName()));
            EXPECT_THAT(stdcout.str(), StrEq("The file specified to write in already exists. Data will be erased before proceeding.\n"));
        }
    }
}


TEST(FileManipulation, ReadAndWriteSimpleFiles)
{
    FileManipulationClassWriter writingToAFile(&myToolBox);
    FileManipulationClassReader readingAFile(&myToolBox);
    std::string data = "I expect these data will be writen in the file.\n";
    writingToAFile.modifyBufferToWrite(data);

    //Test writing while the file is not opened
    EXPECT_THROW(writingToAFile.syncFileWithBuffer(), file_exception);

    //Test reading while the file is not opened
    EXPECT_THROW(readingAFile.syncFileWithBuffer(), file_exception);

    ASSERT_NO_THROW(writingToAFile.openFile("TmpFile.txt")); //file empty
    EXPECT_NO_THROW(writingToAFile.syncFileWithBuffer());
    writingToAFile.closeFile();
    
    EXPECT_NO_THROW(readingAFile.openFile("TmpFile.txt"));
    EXPECT_NO_THROW(readingAFile.syncFileWithBuffer());
    EXPECT_THAT(readingAFile.bufferForReading(), StrEq(data));
    readingAFile.closeFile();
    
    writingToAFile.modifyBufferToWrite("");
    {
        CaptureStream stdcout{std::cout}; //don't want to see std::cout in the test log
        ASSERT_NO_THROW(writingToAFile.openFile("TmpFile.txt")); //file empty
    }
    EXPECT_NO_THROW(writingToAFile.syncFileWithBuffer());
    writingToAFile.closeFile();

    EXPECT_NO_THROW(readingAFile.openFile("TmpFile.txt"));
    EXPECT_NO_THROW(readingAFile.syncFileWithBuffer());
    EXPECT_THAT(readingAFile.bufferForReading(), StrEq(""));
    readingAFile.closeFile();

    remove("TmpFile.txt");
}


TEST(FileManipulation, ReadAndWriteComplexFiles)
{
    std::string nameOfRandomFile = "testfile.dat";
    CreateRandomFile randomFile (nameOfRandomFile, 10, 1);
    size_t sizeOfOriginalFile = myToolBox.returnFileSize(nameOfRandomFile);
    std::string fileForWriting = "testfilecpy.dat";
    FileManipulationClassWriter writingToAFile(&myToolBox);
    FileManipulationClassReader readingAFile(&myToolBox);

    size_t dataRead = 0;
    ASSERT_NO_THROW(readingAFile.openFile(nameOfRandomFile.c_str()));
    ASSERT_NO_THROW(writingToAFile.openFile(fileForWriting.c_str()));


    while (dataRead < sizeOfOriginalFile)
    {
        ASSERT_NO_THROW(readingAFile.syncFileWithBuffer());
        dataRead += readingAFile.getBufferSize();
        writingToAFile.modifyBufferToWrite(readingAFile.getBufferRead());//copy buffer read to buffer write
        ASSERT_NO_THROW(writingToAFile.syncFileWithBuffer());
    }
    writingToAFile.closeFile();
    readingAFile.closeFile();

    ASSERT_THAT(compareFiles(nameOfRandomFile, fileForWriting), IsTrue());

    remove(nameOfRandomFile.c_str());
    remove(fileForWriting.c_str());

}
